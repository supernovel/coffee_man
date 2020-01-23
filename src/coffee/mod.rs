/// 커피 선택 관련 함수  
use serde_json::{json, value};

pub mod route;

// ToDo. 커피 리스트 초기화
// Json 이나 DB에서 가져오면 될 듯?
fn init() {}

// ToDo. 커피 리스트에 추가
fn add_coffee<'a>(name: &str) -> Result<value::Value, &'a str> {
    Ok(json!({
        "blocks": [
            {
                "type": "section",
                "text": {
                    "type": "mrkdwn",
                    "text": name
                }
            }
        ]
    }))
}

// ToDo. 카운트 값을 받으면 결과를 저장할 해쉬맵 초기화, 선택 화면 블록 반화
fn start_pick<'a>(count: u32) -> Result<value::Value, &'a str> {
    Ok(json!({
        "blocks": [
            {
                "type": "section",
                "text": {
                    "type": "mrkdwn",
                    "text": "*It's 80 degrees right now.*"
                }
            },
            {
                "type": "section",
                "text": {
                    "type": "mrkdwn",
                    "text": count
                }
            }
        ]
    }))
}

// ToDo. 커피 선택. 결과 해쉬 맵에 커피 추가 또는 카운트 증가
fn pick_coffee<'a>(name: &str) -> Result<value::Value, &'a str> {
    Ok(json!({
        "blocks": [
            {
                "type": "section",
                "text": {
                    "type": "mrkdwn",
                    "text": name
                }
            }
        ]
    }))
}

// ToDo. 테스트 코드 작성
#[cfg(test)]
mod tests {
    use super::*;

    fn test_start_pick() {
        let result = start_pick(2);
    }
}
