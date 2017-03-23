use std::io;


// impl From<Error> for RedisError {
//     fn from(e: Error) -> RedisError {
//         (redis::ErrorKind::TypeError, "storyestimate error", e.description().to_owned()).into()
//     }
// }

// Create the Error, ErrorKind, ResultExt, and Result types
error_chain! {
    foreign_links {
        IOError(io::Error);
    }
    errors {
        // UserForbidden(t: String) {
        //     description("User attempted operation without proper credentials")
        //         display("{}", t)
        // }
    }
}
