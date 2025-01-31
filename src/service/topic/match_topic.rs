use nucleo_matcher::{Config, Matcher};
use nucleo_matcher::pattern::{CaseMatching, Normalization, Pattern};
use crate::service::topic::svc::TopicService;
use crate::errs::http::Error as HttpError;

impl TopicService {
    pub async fn match_topic(&self, topic: String, size: i32) -> Result<Vec<String>, HttpError> {
        let list = self.topic_list.as_ref().read().map_err(|e| {
            tklog::error!("cannot read topic list, error: ", e);
            HttpError::internal_error(None, None)
        })?;

        let mut matcher = Matcher::new(Config::DEFAULT.match_paths());
        let matches: Vec<(&String, u32)> = Pattern::parse(topic.as_str(), CaseMatching::Ignore, Normalization::Smart).match_list::<&String>(&(*list), &mut matcher);

        let mut result = vec![];
        let end_index = matches.len().min(size as usize);
        for i in 0..end_index {
            let topic = matches[i].0.to_string();
            result.push(topic);
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_match_topic() {
        let topic = String::from("myg");
        let list = vec![
            "MYGO!!!!!".to_string(),
            "BanG Dream It's MyGo".to_string(),
            "it's mygo".to_string(),
        ];

        let mut matcher = Matcher::new(Config::DEFAULT.match_paths());
        let matches: Vec<(&String, u32)> = Pattern::parse(topic.as_str(), CaseMatching::Ignore, Normalization::Smart).match_list::<&String>(&(*list), &mut matcher);

        println!("{:?}", matches);

        let mut result = vec![];
        let end_index = matches.len().min(10);
        for i in 0..end_index {
            let topic = matches[i].0.clone();
            result.push(topic);
        }
        assert_eq!(result.len(), 3);
    }
}