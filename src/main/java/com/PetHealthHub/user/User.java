package com.PetHealthHub.user;
import jakarta.persistence.*;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;

@Getter
@Setter
@NoArgsConstructor
@AllArgsConstructor
@Entity
@Table(name = "users")

// Creating a user's entity for writing information about user to database
public class User {

    private static final long serialVersionUID = 1L;
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Long id;

    @Column(nullable=false, name = "firstname")
    private String firstname;

    @Column(nullable=false, name = "lastname")
    private String lastname;

    @Column(nullable=false, name = "email", unique = true)
    private String email;

    @Column(nullable=false, name = "password")
    private String password;

}
