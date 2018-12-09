/* 
    Module for validate user insances.
    Here we can add validation methods.

    Think, this is better to implement different structs for user instance.
    For example:
        - User has password and email field.
        - User is authenticated only structure.
        - User id is base part of all user actions

        - Profile is just information about user
        - Profile is just object for store user data
        
    Let see on User stuct:
    - password field must be PasswordManager instance
      - PasswordManaher must validate or create password.
      - PasswordManaher must hold inner state.

    - email field must be EmailManager instance
      - EmailManager must validate income email. If it exist in database, or cache.
      - EmailManager must validate email according to email regex

    Think we need define common user instance (adapter) whick can derive different
    user instance from different parts such as database, or cache, or something other.

    For example, If need get user instance from database We have implementation for like `DBUser for User`;
    If need get (user) data from cache, we need to to have implementation for like `CacheUser for User`;

    But, what should code do, when we add User to database or update it??
    Algorithm fot create actions should be:
        - Add user to database
        - Update cache

    But if we update user in database? What should be next?
        - Update cache
        - Update database
    Why it different? Because it already have user_id, so, first we can update faster cache.
    After then we can setup 'task' for update database with new value.

    Ok, How it should implement in code?

    I think, need build UserStructure with fields like:
    ```
    struct User {
        id: Option(usize),
        email: Option(EmailManager) -> Email manager with income raw string
        password: Optional(PasswordManager) -> Password manager with income raw string
        hash: Option(HashManager)
    }
    ``` 

    This structure must have implemetation 'User' with method ::new(raw_email, raw_password).
    Another implementation should be `CreateUser` -> This implementation should Only create user istance anywhere.
    Another implementation should be `UpdateUser` -> This implememtation should Only update user instance anywhrere.
    TODO: Must do boring work, so countinue later...

    Boring work was finished, so I continue work on describing user instance.
    Another implemetation should be `FetchUser` -> This implemetation should Only fetch user instance from anywhere.
    * anywhre - is abstract storage/cache/ladger/etc.
    
    Ok! Let's implement this!
*/


#[cfg(test)]
pub mod validation_tests {
    super::*;

    #[test]
    fn test_should_validate_user() {

    }
}
