package com.PetHealthHub.user.services.implementation;

import com.PetHealthHub.user.User;
import com.PetHealthHub.user.UserDto;
import com.PetHealthHub.user.UserRepository;
import com.PetHealthHub.user.services.UserService;
import org.springframework.security.crypto.password.PasswordEncoder;
import org.springframework.stereotype.Service;


@Service
public class UserServiceImpl implements UserService {
    private UserRepository userRepository;
    private PasswordEncoder passwordEncoder;
    public UserServiceImpl(UserRepository userRepository, PasswordEncoder passwordEncoder){
        this.userRepository = userRepository;
        this.passwordEncoder = passwordEncoder;
    }

    /*
      This method is used to save a user to the database. It takes a UserDto object as input,
      extracts the necessary information, encodes the password,
      and then saves the user to the UserRepository.
     */
    @Override
    public void saveUser(UserDto userDto){
        User user = new User();
        user.setFirstname(userDto.getFirstname());
        user.setLastname(userDto.getLastname());
        user.setEmail(userDto.getEmail());
        user.setPassword(passwordEncoder.encode(userDto.getPassword()));
        userRepository.save(user);
    }

    @Override
    public User findByEmail(String email) {
        return userRepository.findByEmail(email);
    }
}
