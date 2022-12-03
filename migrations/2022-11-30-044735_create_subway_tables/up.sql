-- 전철역 정보
create table if not exists subway_station(
    station_name varchar(30) primary key -- 역 이름
);

-- 전철 노선 정보
create table if not exists subway_route(
    route_id int primary key, -- 노선 ID
    route_name varchar(30) not null -- 노선 이름
);

-- 전철 노선별 역 목록
create table if not exists subway_route_station(
    station_id varchar(10) primary key, -- 역 ID
    route_id int not null, -- 노선 ID
    station_name varchar(30) not null,-- 역 이름
    station_sequence int not null, -- 역 순서
    cumulative_time float not null, -- 누적 시간
    constraint fk_route_id foreign key (route_id) references subway_route(route_id),
    constraint fk_station_id foreign key (station_name) references subway_station(station_name)
);

-- 전철 실시간 운행 정보
create table if not exists subway_realtime(
    station_id varchar(10) not null, -- 역 ID
    arrival_sequence int not null, -- 도착 순서
    current_station_name varchar(30) not null, -- 현재 역 이름
    remaining_stop_count int not null, -- 남은 정류장 수
    remaining_time int not null, -- 남은 시간
    up_down_type varchar(10) not null, -- 상행, 하행 여부
    terminal_station_id varchar(10) not null, -- 종착역 ID
    train_number varchar(10) not null, -- 열차 번호
    last_updated_time timestamp not null, -- 마지막 업데이트 시간
    is_express_train boolean not null, -- 급행 여부
    is_last_train boolean not null, -- 막차 여부
    status_code int not null, -- 상태 코드
    constraint pk_subway_realtime primary key (station_id, up_down_type, arrival_sequence),
    constraint fk_station_id foreign key (station_id) references subway_route_station(station_id),
    constraint fk_terminal_station_id foreign key (terminal_station_id) references subway_route_station(station_id)
);

-- 전철 시간표
create table if not exists subway_timetable(
    station_id varchar(10) not null, -- 역 ID
    terminal_station_id varchar(10) not null, -- 종착역 ID
    departure_time time not null, -- 출발 시간
    weekday varchar(10) not null, -- 평일, 토요일, 일요일 여부
    up_down_type varchar(10) not null, -- 상행, 하행 여부
    constraint pk_subway_timetable primary key (station_id, up_down_type, weekday, departure_time),
    constraint fk_station_id foreign key (station_id) references subway_route_station(station_id),
    constraint fk_terminal_station_id foreign key (terminal_station_id) references subway_route_station(station_id)
);