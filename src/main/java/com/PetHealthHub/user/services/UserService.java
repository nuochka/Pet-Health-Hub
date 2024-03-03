package com.PetHealthHub.user.services;

import com.PetHealthHub.user.User;
import com.PetHealthHub.user.UserDto;

public interface UserService {
    /*
      This method is used to save a user to the database. It takes a UserDto object as input,
      extracts the necessary information, encodes the password,
      and then saves the user to the UserRepository.
     */
    User saveUser(User user);

    User findByEmail(String email);
}
