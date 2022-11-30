-- 통학버스 운행 노선
create table if not exists commute_shuttle_route (
    route_name varchar(15) primary key,
    route_description_korean varchar(100),
    route_description_english varchar(100)
);

-- 통학버스 정류장
create table if not exists commute_shuttle_stop (
    stop_name varchar(50) primary key,
    description varchar(100),
    latitude double precision,
    longitude double precision
);

-- 통학버스 노선별 정류장 순서
create table if not exists commute_shuttle_timetable (
    route_name varchar(15) references commute_shuttle_route(route_name),
    stop_name varchar(50) references commute_shuttle_stop(stop_name),
    stop_order int,
    departure_time timetz not null,
    constraint pk_commute_shuttle_route_stop primary key (route_name, stop_name)
);