use crate::*;

pub async fn fetch_enrolled_courses(session: &Session) -> Result<Vec<Course>> {
    let mut courses = Vec::new();
    let mut ids = std::collections::HashSet::new();

    let mut offset = 0;
    loop {
        let request_body = json!([{
            "args": {
                "classification": "all",
                "limit": 48,
                "offset": offset,
                "sort": "fullname"
            },
            "index": 0,
            "methodname": "core_course_get_enrolled_courses_by_timeline_classification"
        }]);

        let res = session
            .client
            .post(&format!("https://wsdmoodle.waseda.jp/lib/ajax/service.php?sesskey={}&info=core_course_get_enrolled_courses_by_timeline_classification", session.session_key))
            .body(serde_json::to_string(&request_body).unwrap())
            .send()
            .await?
            .text()
            .await?;
        let res_json: serde_json::Value =
            serde_json::from_str(&res).context(ErrorKind::InvalidResponse(None))?;

        let error = res_json[0]["error"].as_bool().unwrap_or(true);
        if error {
            return Err(ErrorKind::InvalidResponse(Some(format!(
                "an error occurred while fetching enrolled courses list: {}",
                res_json[0]["exception"]["message"]
            )))
            .into());
        }

        let courses_json =
            res_json[0]["data"]["courses"]
                .as_array()
                .ok_or(ErrorKind::InvalidResponse(Some(
                    "unknown error occurred while fetching enrolled courses list".to_owned(),
                )))?;
        for course_json in courses_json {
            let course = (|| {
                Some(Course {
                    id: course_json["id"].as_u64()?,
                    name: course_json["fullname"].as_str()?.to_owned(),
                    view_url: course_json["viewurl"].as_str()?.to_owned(),
                    course_image: course_json["courseimage"].as_str()?.to_owned(),
                    progress: if course_json["hasprogress"].as_bool()? {
                        course_json["progress"].as_u64()
                    } else {
                        None
                    },
                    is_favorite: course_json["isfavourite"].as_bool()?,
                })
            })()
            .ok_or(ErrorKind::InvalidResponse(Some(
                "received invalid response while fetching enrolled courses list".to_owned(),
            )))?;

            if !ids.contains(&course.id) {
                ids.insert(course.id);
                courses.push(course);
            }
        }

        let next_offset =
            res_json[0]["data"]["nextoffset"]
                .as_u64()
                .ok_or(ErrorKind::InvalidResponse(Some(
                    "received invalid response while fetching enrolled courses list".to_owned(),
                )))?;
        if offset == next_offset {
            break;
        } else {
            offset = next_offset;
        }
    }

    Ok(courses)
}

#[derive(Debug)]
pub struct Course {
    pub id: u64,
    pub name: String,
    pub view_url: String,
    pub course_image: String,
    pub progress: Option<u64>,
    pub is_favorite: bool,
}
