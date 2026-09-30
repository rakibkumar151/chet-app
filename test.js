fetch("http://localhost:3000/api/v1/auth/signup", {
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify({
    first_name: "Test",
    last_name: "User",
    email: "test_new" + Math.random() + "@example.com",
    gender: "Male",
    password: "testpassword"
  })
}).then(async r => {
  console.log(r.status);
  console.log(await r.text());
});
