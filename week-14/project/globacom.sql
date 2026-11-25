--
-- PostgreSQL database dump
--

\restrict buCOg0uXnKZoInbmF5esl2K4lUN5eaRt0lNUGs3WmanRdqG14hyMy4ojlIBe1Up

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
-- Name: dataplan; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.dataplan (
    data_id integer NOT NULL,
    data_size character varying(10) NOT NULL,
    data_days integer NOT NULL,
    data_price integer NOT NULL
);


ALTER TABLE public.dataplan OWNER TO postgres;

--
-- Name: department; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.department (
    dept_managerid integer NOT NULL,
    dno integer NOT NULL,
    dname text NOT NULL,
    dlocation text NOT NULL,
    pno integer NOT NULL
);


ALTER TABLE public.department OWNER TO postgres;

--
-- Name: project; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.project (
    pno integer NOT NULL,
    pname text NOT NULL,
    pduration text NOT NULL,
    project_managerid integer NOT NULL
);


ALTER TABLE public.project OWNER TO postgres;

--
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    staff_id integer CONSTRAINT employees_eid_not_null NOT NULL,
    staff_name text CONSTRAINT employees_ename_not_null NOT NULL,
    dno integer CONSTRAINT employees_dno_not_null NOT NULL,
    staff_sal integer CONSTRAINT employees_esal_not_null NOT NULL,
    age integer CONSTRAINT employees_age_not_null NOT NULL,
    mobile character varying(11) NOT NULL
);


ALTER TABLE public.staff OWNER TO postgres;

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
-- Data for Name: dataplan; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.dataplan (data_id, data_size, data_days, data_price) FROM stdin;
1	350mb	2	200
2	1.8gb	14	500
3	3.9gb	30	1000
4	7.5gb	30	1500
5	9.2gb	30	2000
6	10.8gb	30	2500
7	14gb	30	3000
8	18gb	30	4000
9	24gb	30	5000
10	29.9gb	30	8000
11	50gb	30	10000
\.


--
-- Data for Name: department; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.department (dept_managerid, dno, dname, dlocation, pno) FROM stdin;
108	1	Administration	Ikeja	44
101	2	Account	Egbeda	11
100	3	Packaging	Ajah	44
120	4	Research	V.I	33
97	5	Account	Magodo	22
122	6	Operations	Mile 2	44
107	7	Packaging	Ketu	55
\.


--
-- Data for Name: project; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.project (pno, pname, pduration, project_managerid) FROM stdin;
11	A	9 Months	102
22	B	14 Months	97
33	C	16 Months	120
44	D	25 Months	108
55	E	9 Months	107
\.


--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (staff_id, staff_name, dno, staff_sal, age, mobile) FROM stdin;
101	Alade Joy	2	250000	33	08023089832
100	Mustapha Ali	3	175000	32	08063285831
107	Alokwe Martin	7	380000	48	07090082812
97	Dankade Aminat	5	550000	40	09023688832
108	Josiah Joshua	1	120000	30	08053189131
102	Mankinde Mary	2	450000	55	09023487830
120	Adeleke Jane	4	200000	38	07061045862
122	Osahon Mark	6	320000	44	08022289842
117	Suleiman Ajayi	3	800000	50	07030089811
104	Kuti Lawal	1	750000	35	09145689842
\.


--
-- Name: customers customer_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customers
    ADD CONSTRAINT customer_pkey PRIMARY KEY (c_id);


--
-- Name: dataplan dataplan_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dataplan
    ADD CONSTRAINT dataplan_pkey PRIMARY KEY (data_id);


--
-- Name: department department_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.department
    ADD CONSTRAINT department_pkey PRIMARY KEY (dept_managerid);


--
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (staff_id);


--
-- Name: project project_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.project
    ADD CONSTRAINT project_pkey PRIMARY KEY (project_managerid);


--
-- PostgreSQL database dump complete
--

\unrestrict buCOg0uXnKZoInbmF5esl2K4lUN5eaRt0lNUGs3WmanRdqG14hyMy4ojlIBe1Up

