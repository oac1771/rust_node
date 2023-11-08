
use rocket::serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Error {
    pub body: String,
}


impl Error {
    pub fn new(err: String) -> Self {
        let response = Error { body: err };

        return response;
    }
}

// pub struct Response<'a, R> {
//         body: &'a str,
//         phantom: PhantomData<R>,
//     }
// impl<R> Response<'_, R> {
    
//     pub fn new<'a, 'de>(body: &'a str) -> Self
//     where
//         R: Deserialize<'de>,
//     {
//         return Response {
//             body,
//             phantom: PhantomData,
//         };
//     }

//     pub fn deserialize<'de>(&self) -> Result<R, Error>
//     where
//         R: Deserialize<'de>,
//     {
//         serde_json::from_str::<R>(self.body).map_err(move |e| Error::new(e.to_string()))
//     }
// }
