package com.PetHealthHub.controllers;

import com.PetHealthHub.user.UserDto;
import com.PetHealthHub.user.services.UserService;
import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.web.bind.annotation.GetMapping;


@Controller
public class AuthController {
    private UserService userService;
    public AuthController(UserService userService) {

        this.userService = userService;
    }
    @GetMapping("/login")
    public String loginForm(){
        return "login";
    }
    @GetMapping("/registration")
    public String registrationForm(Model model){
        UserDto user = new UserDto();
        model.addAttribute("user", user);
        return "registration";
    }
}
