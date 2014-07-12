use tests;

fn setup() {
  let header = Header::new("get", "HTTPS://api.TWITTER.com:443/1/statuses/friendships.json?foo=bar#anchor");
}

test!(stringifies_and_uppercases_the_request_method {
  fail!("Hello")
})

