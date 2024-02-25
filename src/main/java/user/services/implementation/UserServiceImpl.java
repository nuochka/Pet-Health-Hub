package user.services.implementation;

import org.springframework.security.crypto.password.PasswordEncoder;
import user.User;
import user.UserDto;
import user.UserRepository;
import user.services.UserService;

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
        user.setFirstname(user.getFirstname());
        user.setLastname(user.getLastname());
        user.setPassword(passwordEncoder.encode(userDto.getPassword()));
        userRepository.save(user);
    }
}
