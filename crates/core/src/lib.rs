use std::{
    cell::RefCell,
    sync::Arc,
    task::Poll,
    time::{SystemTime, UNIX_EPOCH},
};

use futures::Stream;
use headless_chrome::{Browser, LaunchOptionsBuilder, Tab};

pub struct Fast {
    _browser: Browser,
    tab: Arc<Tab>,
    prev: RefCell<Option<Info>>,
}

#[derive(Clone, Debug)]
pub struct Info {
    last_check: u128,
    pub is_download_done: bool,
    pub is_upload_done: bool,
    pub download_speed: Option<String>,
    pub upload_speed: Option<String>,
    pub download_unit: String,
    pub downloaded: String,
    pub upload_unit: String,
    pub uploaded: String,
    pub latency: String,
    pub buffer_bloat: String,
    pub user_location: String,
    pub user_ip: String,
}

impl Fast {
    pub fn new() -> anyhow::Result<Self> {
        let browser = Browser::new(
            LaunchOptionsBuilder::default()
                .sandbox(false)
                .build()
                .unwrap(),
        )?;

        let tab = browser.wait_for_initial_tab()?;
        tab.navigate_to("https://fast.com")?;

        Ok(Self {
            _browser: browser,
            tab,
            prev: RefCell::new(None),
        })
    }

    fn info(&self) -> anyhow::Result<Info> {
        let is_download_done = self.tab.find_element("#speed-value.succeeded").is_ok();
        let is_upload_done = self.tab.find_element("#upload-value.succeeded").is_ok();
        Ok(Info {
            last_check: SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis(),
            download_speed: Some(
                self.tab
                    .wait_for_element("#speed-value")?
                    .get_inner_text()?,
            ),
            upload_speed: if is_download_done {
                Some(
                    self.tab
                        .wait_for_element("#upload-value")?
                        .get_inner_text()?,
                )
            } else {
                None
            },
            download_unit: self
                .tab
                .wait_for_element("#speed-units")?
                .get_inner_text()?,
            downloaded: self
                .tab
                .wait_for_element("#down-mb-value")?
                .get_inner_text()?,
            upload_unit: self
                .tab
                .wait_for_element("#upload-units")?
                .get_inner_text()?,
            uploaded: self
                .tab
                .wait_for_element("#up-mb-value")?
                .get_inner_text()?,
            latency: self
                .tab
                .wait_for_element("#latency-value")?
                .get_inner_text()?,
            buffer_bloat: self
                .tab
                .wait_for_element("#bufferbloat-value")?
                .get_inner_text()?,
            user_location: self
                .tab
                .wait_for_element("#user-location")?
                .get_inner_text()?,
            user_ip: self.tab.wait_for_element("#user-ip")?.get_inner_text()?,
            is_download_done,
            is_upload_done,
        })
    }
}

impl Info {
    pub fn is_done(&self) -> bool {
        self.is_download_done && self.is_upload_done
    }
}

impl PartialEq for Info {
    fn eq(&self, other: &Self) -> bool {
        self.buffer_bloat == other.buffer_bloat
            && self.download_speed == other.download_speed
            && self.download_unit == other.download_unit
            && self.downloaded == other.downloaded
            && self.latency == other.latency
            && self.upload_speed == other.upload_speed
            && self.upload_unit == other.upload_unit
            && self.uploaded == other.uploaded
    }
}

impl Stream for Fast {
    type Item = Info;
    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let info = self.info().unwrap();

        let prev = self.prev.borrow().clone();

        match prev.as_ref() {
            Some(p) if p.is_done() => {
                return Poll::Ready(None);
            }
            Some(p) if info.last_check - p.last_check < 100 => return Poll::Pending,
            _ => {
                self.prev.borrow_mut().replace(info.clone());
                Poll::Ready(Some(info))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use futures::StreamExt;

    use super::*;

    #[tokio::test]
    async fn it_works() -> anyhow::Result<()> {
        let mut fast = Fast::new()?;

        let info = fast.next().await;

        println!("{:?}", info);

        Ok(())
    }
}
