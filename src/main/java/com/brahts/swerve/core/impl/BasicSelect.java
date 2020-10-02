package com.brahts.swerve.core.impl;

import com.brahts.swerve.core.From;
import com.brahts.swerve.core.Select;

public class BasicSelect implements Select {
    @Override
    public From from() {
        return new BasicFrom();
    }
}
