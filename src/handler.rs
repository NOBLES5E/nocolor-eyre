use crate::{
    config::BacktraceFormatter,
    section::help::HelpInfo,
    writers::{EnvSection, WriterExt},
    Handler,
};
use backtrace::Backtrace;
use indenter::{indented, Format};
use std::fmt::Write;

impl std::fmt::Debug for Handler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("redacted")
    }
}

impl Handler {
    /// Return a reference to the captured `Backtrace` type
    pub fn backtrace(&self) -> Option<&Backtrace> {
        self.backtrace.as_ref()
    }

    pub(crate) fn format_backtrace<'a>(
        &'a self,
        trace: &'a backtrace::Backtrace,
    ) -> BacktraceFormatter<'a> {
        BacktraceFormatter {
            filters: &self.filters,
            inner: trace,
        }
    }
}

impl eyre::EyreHandler for Handler {
    fn debug(
        &self,
        error: &(dyn std::error::Error + 'static),
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        if f.alternate() {
            return core::fmt::Debug::fmt(error, f);
        }

        let errors = || eyre::Chain::new(error).enumerate();

        for (n, error) in errors() {
            writeln!(f)?;
            write!(indented(f).ind(n), "{}", error)?;
        }

        let mut separated = f.header("\n\n");

        #[cfg(feature = "track-caller")]
        if self.display_location_section {
            write!(
                separated.ready(),
                "{}",
                crate::SectionExt::header(crate::fmt::LocationSection(self.location), "Location:")
            )?;
        }

        for section in self
            .sections
            .iter()
            .filter(|s| matches!(s, HelpInfo::Error(_)))
        {
            write!(separated.ready(), "{}", section)?;
        }

        for section in self
            .sections
            .iter()
            .filter(|s| matches!(s, HelpInfo::Custom(_)))
        {
            write!(separated.ready(), "{}", section)?;
        }

        if !self.suppress_backtrace {
            if let Some(backtrace) = self.backtrace.as_ref() {
                let fmted_bt = self.format_backtrace(backtrace);

                write!(
                    indented(&mut separated.ready())
                        .with_format(Format::Uniform { indentation: "  " }),
                    "{}",
                    fmted_bt
                )?;
            }
        }

        let f = separated.ready();
        let mut h = f.header("\n");
        let mut f = h.in_progress();

        for section in self
            .sections
            .iter()
            .filter(|s| !matches!(s, HelpInfo::Custom(_) | HelpInfo::Error(_)))
        {
            write!(&mut f, "{}", section)?;
            f = h.ready();
        }

        if self.display_env_section {
            let env_section = EnvSection {
                bt_captured: &self.backtrace.is_some(),
            };

            write!(&mut separated.ready(), "{}", env_section)?;
        }

        #[cfg(feature = "issue-url")]
        if self.issue_url.is_some() && (*self.issue_filter)(crate::ErrorKind::Recoverable(error)) {
            let url = self.issue_url.as_ref().unwrap();
            let mut payload = String::from("Error: ");
            for (n, error) in errors() {
                writeln!(&mut payload)?;
                write!(indented(&mut payload).ind(n), "{}", error)?;
            }

            let issue_section = crate::section::github::IssueSection::new(url, &payload)
                .with_backtrace(self.backtrace.as_ref())
                .with_metadata(&**self.issue_metadata);

            write!(&mut separated.ready(), "{}", issue_section)?;
        }

        Ok(())
    }

    #[cfg(feature = "track-caller")]
    fn track_caller(&mut self, location: &'static std::panic::Location<'static>) {
        self.location = Some(location);
    }
}
