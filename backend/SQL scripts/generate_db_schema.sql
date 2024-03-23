create table if not exists users
(
	email varchar not null
		constraint users_pk
			primary key,
	first_name varchar not null,
	last_name varchar not null
);

alter table users owner to postgres;

create table if not exists submissions
(
	powersort_comps json,
	timsort_comps json,
	email varchar not null
		constraint submissions_fk
			references users
);

alter table submissions owner to postgres;


