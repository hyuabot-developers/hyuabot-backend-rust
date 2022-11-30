-- 셔틀버스 운행 기간 종류
create table if not exists shuttle_period_type (
    period_type varchar(20) primary key
);

-- 셔틀버스 운행 노선
create table if not exists shuttle_route (
    route_name varchar(15) primary key,
    route_description_korean varchar(100),
    route_description_english varchar(100)
);

-- 셔틀버스 정류장
create table if not exists shuttle_stop (
    stop_name varchar(15) primary key,
    latitude double precision,
    longitude double precision
);

-- 셔틀버스 노선별 정류장 순서
create table if not exists shuttle_route_stop (
    route_name varchar(15) references shuttle_route(route_name),
    stop_name varchar(15) references shuttle_stop(stop_name),
    stop_order int,
    cumulative_time int,
    constraint pk_shuttle_route_stop primary key (route_name, stop_name)
);

-- 셔틀버스 운행 기간 (학기중, 계절학기, 방학)
create table if not exists shuttle_period(
    -- 셔틀버스 운행 기간 ID
    period_type varchar(20) not null,
    period_start timestamptz not null,
    period_end timestamptz not null,
    constraint pk_shuttle_period primary key (period_type, period_start, period_end),
    constraint fk_period_type foreign key (period_type) references shuttle_period_type(period_type)
);

-- 셔틀버스 운행 시간표
create table if not exists shuttle_timetable(
    period_type varchar(20) not null,
    weekday boolean not null, -- 평일 여부
    route_name varchar(15) not null,
    departure_time timetz not null,
    start_stop varchar(15) not null,
    constraint pk_shuttle_timetable primary key (period_type, weekday, route_name, departure_time),
    constraint fk_period_type foreign key (period_type) references shuttle_period_type(period_type),
    constraint fk_route_name foreign key (route_name) references shuttle_route(route_name),
    constraint fk_start_stop foreign key (start_stop) references shuttle_stop(stop_name)
);

-- 셔틀 임시 휴일
create table if not exists shuttle_holiday(
    holiday_date date not null,
    holiday_type varchar(15) not null,
    calendar_type varchar(15) not null,
    constraint pk_shuttle_holiday primary key (holiday_date, holiday_type, calendar_type)
);