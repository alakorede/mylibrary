To publicate your library on crates, go to crates.io

1. Create a login with your github account

2. Access Dashboard >> Account Settings >> API Tokens

3. Create a New Token, name it

4. On token creation, select to publish-new library

5. The Crate Name must be the same name user on your library

6. Inside the library's root directory run : $ cargo login

7. Add the token created on crates website

8. $ cargo publish --dry-run (To simulate if you can really publicate your library)

9. Fix the requirements if necessary.

10. $ cargo publish
