package com.PetHealthHub.controllers;

import com.PetHealthHub.user.User;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.ModelAttribute;
import org.springframework.web.bind.annotation.SessionAttributes;
import org.springframework.web.bind.support.SessionStatus;

@Controller
@SessionAttributes("user")
public class SessionController {
    @ModelAttribute("user")
    public User setUpUser(){
        return new User();
    }

    @GetMapping("/logout")
    public String logout(@ModelAttribute("user") User user, SessionStatus sessionStatus){
        sessionStatus.setComplete();
        return "redirect:/login";
    }
}