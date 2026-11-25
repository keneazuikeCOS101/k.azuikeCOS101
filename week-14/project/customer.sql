--
-- PostgreSQL database dump
--

\restrict kZy8IooUSv5FqbqJ8UM64Dfqly3WdVg0TiOYClmLt4usulwh7nli0sAIVA8HteB

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
-- Name: customers; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customers (
    c_id integer CONSTRAINT customer_c_id_not_null NOT NULL,
    c_name character varying(50) CONSTRAINT customer_c_name_not_null NOT NULL,
    c_age integer CONSTRAINT customer_c_age_not_null NOT NULL,
    c_email character varying(50) CONSTRAINT customer_c_email_not_null NOT NULL,
    c_mobile character varying(50) CONSTRAINT customer_c_mobile_not_null NOT NULL,
    eid integer,
    data_id integer
);


ALTER TABLE public.customers OWNER TO postgres;

--
-- Data for Name: customers; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customers (c_id, c_name, c_age, c_email, c_mobile, eid, data_id) FROM stdin;
110	Mustu Karim	35	m_karim@gmail.com	0805589112	102	5
111	Lilian Jaiya	43	l_jaiya@gmail.com	0805513541	100	3
112	Arthur Musa	50	a_musa@gmail.com	0705522813	107	10
113	Philip Akonjo	41	p_akonjo@gmail.com	09052356772	100	2
114	Marylene Mapa	33	m_mapa@gmail.com	08053333551	120	5
115	Oghenero Agor	50	o_agor@gmail.com	07055566774	117	11
116	Adams Brae	33	a_brae@gmail.com	08056765424	102	1
117	Okafor Mathias	45	o_mathias@gmail.com	08056763367	120	10
118	Samson Adeleke	65	s_adelke@gmail.com	07056274423	117	11
119	Lawal Tamine	35	l_tamine@gmail.com	09052111101	107	5
120	James Job	44	j_job@gmail.com	08059639319	100	8
121	Matthew Jakande	21	m_jakande@gmail.com	07051232144	120	2
122	Jimila Adegboye	20	j_adegboye@gmail.com	08054921923	107	5
\.


--
-- Name: customers customer_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customers
    ADD CONSTRAINT customer_pkey PRIMARY KEY (c_id);


--
-- PostgreSQL database dump complete
--

\unrestrict kZy8IooUSv5FqbqJ8UM64Dfqly3WdVg0TiOYClmLt4usulwh7nli0sAIVA8HteB

