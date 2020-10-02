package com.brahts.swerve.core.impl;

import com.brahts.swerve.core.From;
import com.brahts.swerve.core.Result;

public class BasicFrom implements From {
    @Override
    public Result fetch() {
        return new BasicResult();
    }
}
