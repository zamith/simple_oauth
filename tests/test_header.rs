use simple_oauth::Header;

fn header() -> Header {
  Header::new("get", "HTTPS://api.TWITTER.com:443/1/statuses/friendships.json?foo=bar#anchor")
}

test!(stringifies_and_uppercases_the_request_method {
  assert_eq!(header().method().as_slice(), "GET");
})

test!(downcases_the_scheme_and_authority {
  let re = regex!(r"^https://api.twitter.com");
  assert!(re.is_match(header().url().as_slice()));
})
