package com.PetHealthHub.controllers;

import com.PetHealthHub.user.User;
import com.PetHealthHub.user.UserDto;
import com.PetHealthHub.user.UserRepository;
import com.PetHealthHub.user.services.UserService;
import jakarta.servlet.http.HttpSession;
import jakarta.validation.Valid;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.security.core.Authentication;
import org.springframework.security.core.context.SecurityContextHolder;
import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.validation.BindingResult;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.ModelAttribute;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestParam;

@Controller
public class AuthController {
    private UserService userService;
    @Autowired
    private UserRepository userRepository;

    public AuthController(UserService userService) {
        this.userService = userService;
    }

    @GetMapping("/login")
    public String loginForm() {
        return "login";
    }

//    @PostMapping("/login")
//    public String login(@RequestParam("username") String email, Model model) {
//        User user = userRepository.findByEmail(email);
//        System.out.println(email);
//        if (user == null) {
//            model.addAttribute("error", "Invalid email or password");
//            return "/login";
//        }
//        return "redirect:/main-page";
//    }

    @GetMapping("/registration")
    public String registrationForm(Model model) {
        UserDto user = new UserDto();
        model.addAttribute("user", user);
        return "registration";
    }

    @GetMapping("/welcome")
    public String welcomePage() {
        return "welcome";
    }

    @GetMapping("/")
    public String redirectWelcomePage() {
        return "redirect:/welcome";
    }

    @PostMapping("/registration/save")
    public String registration(@Valid @ModelAttribute("user") User user,
                               BindingResult result,
                               Model model) {
        User existing = userService.findByEmail(user.getEmail());
        if (existing != null) {
            result.rejectValue("email", "There is already an account registered with that email");
        }
        if (result.hasErrors()) {
            model.addAttribute("user", user);
            return "registration";
        }
        userService.saveUser(user);
        return "redirect:/registration?success";
    }

    @GetMapping("/main-page")
    public String mainPage() {
        return "mainPage";
    }
}
