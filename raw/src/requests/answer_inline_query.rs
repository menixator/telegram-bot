use crate::requests::*;
use crate::types::*;

#[derive(Serialize, Debug)]
pub struct AnswerInlineQuery {
    inline_query_id: InlineQueryId,
    results: Vec<InlineQueryResult>,
    // TODO: Rest of the fields
    next_offset: Option<String>,
}


impl Request for AnswerInlineQuery {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("answerInlineQuery"), self)
    }
}

pub trait CanAnswerInlineQuery {
    fn answer(self, results: Vec<InlineQueryResult>) -> AnswerInlineQuery;
}

impl<T> CanAnswerInlineQuery for T
where
    T: Into<InlineQueryId>,
{
    fn answer(self, results: Vec<InlineQueryResult>) -> AnswerInlineQuery {
        AnswerInlineQuery::new(self.into(), results, None)
    }
}

impl AnswerInlineQuery {
    pub fn new(
        inline_query_id: InlineQueryId,
        results: Vec<InlineQueryResult>,
        next_offset: Option<String>
    ) -> AnswerInlineQuery {
        AnswerInlineQuery {
            inline_query_id,
            results,
            next_offset
        }
    }

    pub fn add_inline_result<T: Into<InlineQueryResult>>(&mut self, result: T) {
        self.results.push(result.into());
    }
}
