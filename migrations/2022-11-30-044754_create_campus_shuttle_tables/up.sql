-- 캠퍼스
create table if not exists campus(
    campus_id int primary key, -- 캠퍼스 ID
    campus_name varchar(30) not null -- 캠퍼스 이름
);

-- 학식을 제공하는 식당
create table if not exists restaurant(
    campus_id int not null, -- 캠퍼스 ID
    restaurant_id int primary key, -- 식당 ID
    restaurant_name varchar(50) not null, -- 식당 이름
    latitude double precision not null, -- 식당 위도
    longitude double precision not null, -- 식당 경도
    constraint fk_campus_id foreign key (campus_id) references campus(campus_id)
);

-- 학식 메뉴
create table if not exists menu(
    restaurant_id int not null, -- 식당 ID
    time_type varchar(10) not null, -- 시간 타입 (아침, 점심, 저녁)
    menu_name varchar(100) not null, -- 메뉴 이름
    menu_price varchar(30) not null, -- 메뉴 가격
    constraint pk_menu primary key (restaurant_id, time_type, menu),
    constraint fk_restaurant_id foreign key (restaurant_id) references restaurant(restaurant_id)
);

-- 열람실 정보
create table if not exists reading_room(
    campus_id int not null, -- 캠퍼스 ID
    room_id int primary key, -- 열람실 ID
    room_name varchar(30) not null, -- 열람실 이름
    is_active boolean not null, -- 열람실 활성화 여부
    is_reservable boolean not null, -- 열람실 예약 가능 여부
    total int not null, -- 열람실 총 좌석 수
    active_total int not null, -- 열람실 활성화된 좌석 수
    occupied int not null, -- 열람실 사용중인 좌석 수
    available int not null, -- 열람실 사용 가능한 좌석 수
    constraint fk_campus_id foreign key (campus_id) references campus(campus_id)
);