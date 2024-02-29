package com.PetHealthHub.user.services;

import com.PetHealthHub.user.User;
import com.PetHealthHub.user.UserDto;

public interface UserService {

    void saveUser(UserDto userDto);

    User findByEmail(String email);
}
