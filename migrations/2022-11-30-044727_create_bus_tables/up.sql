-- 버스 정류장
create table if not exists bus_stop (
    stop_id int primary key, -- 정류장 ID(GBIS)
    stop_name varchar(30), -- 정류장 이름
    district_code int not null, -- 인가기관 코드
    mobile_number varchar(15) not null, -- 정류장 검색 ID(숫자 5자리)
    region_name varchar(10) not null, -- 지역명
    latitude double precision not null, -- 정류장 위도
    longitude double precision not null -- 정류장 경도
);

-- 버스 노선
create table if not exists bus_route (
    -- 운행사 정보
    company_id int,
    company_name varchar(30) not null,
    company_telephone varchar(15) not null,
    -- 관리 기관 정보
    district_code int not null,
    -- 평일 기점 → 종점 방면 첫차, 막차
    up_first_time time not null,
    up_last_time time not null,
    -- 평일 종점 → 기점 방면 첫차, 막차
    down_first_time time not null,
    down_last_time time not null,
    -- 기점 정류소
    start_stop_id int not null,
    -- 종점 정류소
    end_stop_id int not null,
    -- 노선 정보
    route_id int primary key, -- 노선 ID(GBIS)
    route_name varchar(30) not null, -- 노선 이름
    route_type_code varchar(10) not null, -- 노선 유형
    route_type_name varchar(10) not null, -- 노선 유형 이름
    -- FK
    constraint fk_start_stop_id foreign key (start_stop_id) references bus_stop(stop_id),
    constraint fk_end_stop_id foreign key (end_stop_id) references bus_stop(stop_id)
);

-- 각 노선별 경유 정류장 목록 조회
create table if not exists bus_route_stop (
    route_id int not null,
    stop_id int not null,
    stop_sequence int not null,
    constraint pk_bus_route_stop primary key (route_id, stop_id),
    constraint fk_route_id foreign key (route_id) references bus_route(route_id),
    constraint fk_stop_id foreign key (stop_id) references bus_stop(stop_id)
);

-- 버스 실시간 운행 정보
create table if not exists bus_realtime(
    stop_id int not null, -- 정류장 ID
    route_id int not null, -- 노선 ID
    arrival_sequence int not null, -- 도착 순서
    remaining_stop_count int not null, -- 남은 정류장 수
    remaining_seat_count int not null, -- 남은 좌석 수
    remaining_time int not null, -- 남은 시간
    low_plate boolean not null, -- 저상 버스 여부
    constraint pk_bus_realtime primary key (stop_id, route_id, arrival_sequence),
    constraint fk_station_id foreign key (stop_id) references bus_stop(stop_id),
    constraint fk_route_id foreign key (route_id) references bus_route(route_id)
);

-- 버스 회차지 출발 시간표
create table if not exists bus_timetable(
    route_id int not null, -- 노선 ID
    start_stop_id int not null, -- 기점 정류장 ID
    departure_time time not null, -- 출발 시간
    weekday varchar(10) not null, -- 평일, 토요일, 일요일 여부
    constraint pk_bus_timetable primary key (route_id, start_stop_id, departure_time, weekday),
    constraint fk_route_id foreign key (route_id) references bus_route(route_id),
    constraint fk_start_stop_id foreign key (start_stop_id) references bus_stop(stop_id)
);