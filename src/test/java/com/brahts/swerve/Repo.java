package com.brahts.swerve;

import com.brahts.swerve.core.Selector;
import org.springframework.data.jpa.repository.JpaRepository;

import javax.persistence.Id;

public interface Repo extends JpaRepository<Selector, Id> {
}
