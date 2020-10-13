use chrono::prelude::*;
use colored::*;
use serde::{Deserialize, Serialize};
static LOGO: &str = "\n.......................................................................................\n.......................................................................................\n...................................::....:***:....:::..................................\n............................***:::*FFF***FFFFF***FFF*:::***............................\n.....................:::::.:FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF*.:::::.....................\n.....................:FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF*.....................\n................:*****FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF******................\n................:FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF:................\n...........:*****FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF*****:...........\n............:FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF*............\n........::****FFFFFFFFFFFFFFFFFFFF*****FFFFFFFFF*****FFFFFFFFFFFFFFFFFFFFF***::........\n.........*FFFFFFFFFFFFFFFFFFFFF*:......:*FFFFF*:......:*FFFFFFFFFFFFFFFFFFFFF*:........\n.......::**FFFFFFFFFFFFFFFFFFF*:..:::...:FFFFF:...:::..:*FFFFFFFFFFFFFFFFFFFF*::.......\n.....:*FFFFFFFFFFFFFFFFFFFFFFF*:.INNM*..*FFFFF*:.*MNNV.:*FFFFFFFFFFFFFFFFFFFFFFF*:.....\n.....:FFFFFF******FFFFFFFFFFFFFF*:**::****FFFFFF*::**:**FFFFFFFFFFFFFF*****FFFFFF:.....\n......*FFFFFF*******FFF*****FFFFFFFFFFF*FVVVIF**FFFFFFFFFFF*****FFF*******FFFFFF*......\n.......:*FFFF*::*****FF***::::***************************:::::**FF*****::*FFFF*:.......\n.........:*FFF*...:*::**FF****FFFFFFF**::......:**FFFFFFF****FF**:.*::..*FFF*:.........\n...........:**F*........::*FFFFFFFFFFFFF*:...:*FFFFFFFFFFFFF*::........*F**:...........\n..............:*:.........:*FFFFFF******F*:::*F***F***FFFFF*:.........:*:..............\n............................:*FFFF***F*FFFFIFIFF*F***FFFF*::...........................\n...............................:*F*****FI$$MMM$IF****F*:...............................\n...............................:***IFVIF$M$$$MMVIVVVF$F*...............................\n..............................*I$**$FIFF$I$M$IVIFFIVVF$V*..............................\n..............................**FFFMMIFVMVV$VV$VIFMM$FF**..............................\n.................................:****IV$$$$$$$VV***::.................................\n.......................................................................................";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Filter {
  Info,
  Warn,
  Error,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Logger {
  pub enabled: bool,
  pub filter: Vec<Filter>,
}

impl Logger {
  pub fn new(enabled: bool, filter: Vec<Filter>) -> Logger {
    println!("{}", LOGO.blue());
    Logger { enabled, filter }
  }
  pub fn info(&self, content: String) {
    if !&self.enabled || self.filter.contains(&Filter::Info) {
      ();
    }
    println!("{} [{}] {}", "INFO".blue().bold(), Utc::now(), content);
  }

  pub fn warn(&self, content: String) {
    if !&self.enabled || self.filter.contains(&Filter::Warn) {
      ();
    }
    println!("{} [{}] {}", Utc::now(), "WARN".yellow().bold(), content);
  }

  pub fn err(&self, content: String) {
    if !&self.enabled || self.filter.contains(&Filter::Error) {
      ();
    }
    println!("{} [{}] {}", Utc::now(), "ERROR".red().bold(), content)
  }
}
