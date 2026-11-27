--
-- PostgreSQL database dump
--

\restrict cj1QwfrPNi3YVwhEeUtfNJO2ojOV6aMdhLKzaBDjIAfl9qvZYggX8B6GgJvK2Zj

-- Dumped from database version 18.6
-- Dumped by pg_dump version 18.6

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: dataplan; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.dataplan (
    data_id integer NOT NULL,
    data_size character(50) NOT NULL,
    data_duration integer NOT NULL,
    data_price integer NOT NULL
);


ALTER TABLE public.dataplan OWNER TO postgres;

--
-- PostgreSQL database dump complete
--

\unrestrict cj1QwfrPNi3YVwhEeUtfNJO2ojOV6aMdhLKzaBDjIAfl9qvZYggX8B6GgJvK2Zj

