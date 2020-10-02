package com.brahts.swerve;

import com.brahts.swerve.core.Query;
import com.brahts.swerve.core.Result;
import com.brahts.swerve.core.Selector;
import com.brahts.swerve.core.impl.BasicQuery;
import lombok.extern.slf4j.Slf4j;
import org.junit.jupiter.api.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;

import java.util.List;

import static com.brahts.swerve.core.Query.select;

@SpringBootTest
@Slf4j
class SwerveApplicationTests {
	@Autowired
	private Repo repo;

	@Test
	void contextLoads() {
	}

	@Test
	void whatever() {

		log.info(repo.save(new Selector()).toString());
	}

	@Test
	void goal() {
		Result query = select().from().fetch();
	}
}
