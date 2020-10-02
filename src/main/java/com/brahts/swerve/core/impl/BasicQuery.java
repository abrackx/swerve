package com.brahts.swerve.core.impl;

import com.brahts.swerve.core.From;
import com.brahts.swerve.core.Query;
import com.brahts.swerve.core.Result;
import com.brahts.swerve.core.Select;

public class BasicQuery implements Query {
    public static Select select() {
        return () -> (From) () -> new Result(){

        };
    }
}
