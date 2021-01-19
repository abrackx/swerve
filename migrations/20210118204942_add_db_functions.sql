CREATE OR REPLACE FUNCTION add_user_project(
    name TEXT,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE,
    modified_at TIMESTAMP WITH TIME ZONE,
    user_id INTEGER
) RETURNS INTEGER AS $$
    DECLARE project_id INTEGER;
    BEGIN
        INSERT INTO projects(name, description, created_at, modified_at)
        VALUES (name, description, created_at, modified_at) RETURNING id INTO project_id;

        INSERT INTO user_projects(user_id, project_id)
        VALUES ($5, project_id);

        RETURN project_id;
    END
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION add_project_file(
    name TEXT,
    created_at TIMESTAMP WITH TIME ZONE,
    modified_at TIMESTAMP WITH TIME ZONE,
    project_id INTEGER
) RETURNS INTEGER AS $$
    DECLARE file_id INTEGER;
    BEGIN
        INSERT INTO files(name, created_at, modified_at)
        VALUES (name, created_at, modified_at) RETURNING id INTO file_id;

        INSERT INTO project_files(file_id, project_id)
        VALUES (file_id, project_id);

        RETURN file_id;
    END
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION add_project_image(
    name TEXT,
    created_at TIMESTAMP WITH TIME ZONE,
    modified_at TIMESTAMP WITH TIME ZONE,
    project_id INTEGER
) RETURNS INTEGER AS $$
    DECLARE image_id INTEGER;
    BEGIN
        INSERT INTO images(name, created_at, modified_at)
        VALUES (name, created_at, modified_at) RETURNING id INTO image_id;

        INSERT INTO project_images(image_id, project_id)
        VALUES (image_id, project_id);

        RETURN image_id;
    END
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION add_project_tag(
    name TEXT,
    created_at TIMESTAMP WITH TIME ZONE,
    project_id INTEGER
) RETURNS INTEGER AS $$
    DECLARE tag_id INTEGER;
    BEGIN
        INSERT INTO tags(name, created_at)
        VALUES (name, created_at) RETURNING id INTO tag_id;

        INSERT INTO project_images(tag_id, project_id)
        VALUES (tag_id, project_id);

        RETURN tag_id;
    END
$$ LANGUAGE plpgsql;