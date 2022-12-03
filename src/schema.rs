// @generated automatically by Diesel CLI.

diesel::table! {
    bus_realtime (stop_id, route_id, arrival_sequence) {
        stop_id -> Int4,
        route_id -> Int4,
        arrival_sequence -> Int4,
        remaining_stop_count -> Int4,
        remaining_seat_count -> Int4,
        remaining_time -> Int4,
        low_plate -> Bool,
    }
}

diesel::table! {
    bus_route (route_id) {
        company_id -> Nullable<Int4>,
        company_name -> Varchar,
        company_telephone -> Varchar,
        district_code -> Int4,
        up_first_time -> Time,
        up_last_time -> Time,
        down_first_time -> Time,
        down_last_time -> Time,
        start_stop_id -> Int4,
        end_stop_id -> Int4,
        route_id -> Int4,
        route_name -> Varchar,
        route_type_code -> Varchar,
        route_type_name -> Varchar,
    }
}

diesel::table! {
    bus_route_stop (route_id, stop_id) {
        route_id -> Int4,
        stop_id -> Int4,
        stop_sequence -> Int4,
    }
}

diesel::table! {
    bus_stop (stop_id) {
        stop_id -> Int4,
        stop_name -> Nullable<Varchar>,
        district_code -> Int4,
        mobile_number -> Varchar,
        region_name -> Varchar,
        latitude -> Float8,
        longitude -> Float8,
    }
}

diesel::table! {
    bus_timetable (route_id, start_stop_id, departure_time, weekday) {
        route_id -> Int4,
        start_stop_id -> Int4,
        departure_time -> Time,
        weekday -> Varchar,
    }
}

diesel::table! {
    campus (campus_id) {
        campus_id -> Int4,
        campus_name -> Varchar,
    }
}

diesel::table! {
    commute_shuttle_route (route_name) {
        route_name -> Varchar,
        route_description_korean -> Nullable<Varchar>,
        route_description_english -> Nullable<Varchar>,
    }
}

diesel::table! {
    commute_shuttle_stop (stop_name) {
        stop_name -> Varchar,
        description -> Nullable<Varchar>,
        latitude -> Nullable<Float8>,
        longitude -> Nullable<Float8>,
    }
}

diesel::table! {
    commute_shuttle_timetable (route_name, stop_name) {
        route_name -> Varchar,
        stop_name -> Varchar,
        stop_order -> Nullable<Int4>,
        departure_time -> Time,
    }
}

diesel::table! {
    menu (restaurant_id, time_type, menu_name) {
        restaurant_id -> Int4,
        time_type -> Varchar,
        menu_name -> Varchar,
        menu_price -> Varchar,
    }
}

diesel::table! {
    reading_room (room_id) {
        campus_id -> Int4,
        room_id -> Int4,
        room_name -> Varchar,
        is_active -> Bool,
        is_reservable -> Bool,
        total -> Int4,
        active_total -> Int4,
        occupied -> Int4,
        available -> Int4,
    }
}

diesel::table! {
    restaurant (restaurant_id) {
        campus_id -> Int4,
        restaurant_id -> Int4,
        restaurant_name -> Varchar,
        latitude -> Float8,
        longitude -> Float8,
    }
}

diesel::table! {
    shuttle_holiday (holiday_date, holiday_type, calendar_type) {
        holiday_date -> Date,
        holiday_type -> Varchar,
        calendar_type -> Varchar,
    }
}

diesel::table! {
    shuttle_period (period_type, period_start, period_end) {
        period_type -> Varchar,
        period_start -> Timestamp,
        period_end -> Timestamp,
    }
}

diesel::table! {
    shuttle_period_type (period_type) {
        period_type -> Varchar,
    }
}

diesel::table! {
    shuttle_route (route_name) {
        route_name -> Varchar,
        route_description_korean -> Nullable<Varchar>,
        route_description_english -> Nullable<Varchar>,
    }
}

diesel::table! {
    shuttle_route_stop (route_name, stop_name) {
        route_name -> Varchar,
        stop_name -> Varchar,
        stop_order -> Nullable<Int4>,
        cumulative_time -> Nullable<Int4>,
    }
}

diesel::table! {
    shuttle_stop (stop_name) {
        stop_name -> Varchar,
        latitude -> Nullable<Float8>,
        longitude -> Nullable<Float8>,
    }
}

diesel::table! {
    shuttle_timetable (period_type, weekday, route_name, departure_time) {
        period_type -> Varchar,
        weekday -> Bool,
        route_name -> Varchar,
        departure_time -> Time,
        start_stop -> Varchar,
    }
}

diesel::table! {
    subway_realtime (station_id, up_down_type, arrival_sequence) {
        station_id -> Varchar,
        arrival_sequence -> Int4,
        current_station_name -> Varchar,
        remaining_stop_count -> Int4,
        remaining_time -> Int4,
        up_down_type -> Varchar,
        terminal_station_id -> Varchar,
        train_number -> Varchar,
        last_updated_time -> Timestamp,
        is_express_train -> Bool,
        is_last_train -> Bool,
        status_code -> Int4,
    }
}

diesel::table! {
    subway_route (route_id) {
        route_id -> Int4,
        route_name -> Varchar,
    }
}

diesel::table! {
    subway_route_station (station_id) {
        station_id -> Varchar,
        route_id -> Int4,
        station_name -> Varchar,
        station_sequence -> Int4,
        cumulative_time -> Float8,
    }
}

diesel::table! {
    subway_station (station_name) {
        station_name -> Varchar,
    }
}

diesel::table! {
    subway_timetable (station_id, up_down_type, weekday, departure_time) {
        station_id -> Varchar,
        terminal_station_id -> Varchar,
        departure_time -> Time,
        weekday -> Varchar,
        up_down_type -> Varchar,
    }
}

diesel::joinable!(bus_route_stop -> bus_stop (stop_id));
diesel::joinable!(commute_shuttle_timetable -> commute_shuttle_route (route_name));
diesel::joinable!(commute_shuttle_timetable -> commute_shuttle_stop (stop_name));
diesel::joinable!(menu -> restaurant (restaurant_id));
diesel::joinable!(shuttle_route_stop -> shuttle_route (route_name));
diesel::joinable!(shuttle_route_stop -> shuttle_stop (stop_name));
diesel::joinable!(shuttle_timetable -> shuttle_route (route_name));
diesel::joinable!(shuttle_timetable -> shuttle_stop (start_stop));

diesel::allow_tables_to_appear_in_same_query!(
    bus_realtime,
    bus_route,
    bus_route_stop,
    bus_stop,
    bus_timetable,
    campus,
    commute_shuttle_route,
    commute_shuttle_stop,
    commute_shuttle_timetable,
    menu,
    reading_room,
    restaurant,
    shuttle_holiday,
    shuttle_period,
    shuttle_period_type,
    shuttle_route,
    shuttle_route_stop,
    shuttle_stop,
    shuttle_timetable,
    subway_realtime,
    subway_route,
    subway_route_station,
    subway_station,
    subway_timetable,
);
