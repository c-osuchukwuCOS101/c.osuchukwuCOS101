pno |                        name                        | duration  | project_managerid
-----+----------------------------------------------------+-----------+-------------------
  11 | A                                                  | 9 Months  |               102
  22 | B                                                  | 14 Months |                97
  33 | C                                                  | 16 Months |               120
  44 | D                                                  | 25 Months |               108
  55 | E                                                  | 9 Months  |               107
(5 rows)


globacom_dbase=# SELECT * FROM CUSTOMER;
 c_id |     c_name      | c_age |                      c_email                       |  c_mobile  | eid | data_id
------+-----------------+-------+----------------------------------------------------+------------+-----+---------
  110 | Musta Karim     |    25 | m_karim@gmail.com                                  | 8055089112 | 102 |       5
  111 | Lilian Jaiya    |    43 | l_jaiya@gmail.com                                  | 8055185341 | 100 |       3
  112 | Arthur Musa     |    50 | a_musa@gmail.com                                   | 7055282813 | 107 |      10
  113 | Philip Akonjo   |    41 | p_akonjo@gmail.com                                 | 9052356772 | 100 |       2
  114 | Marylene Mapa   |    22 | m_mapa@gmail.com                                   | 8053333551 | 120 |       5
  115 | Oghenero Agor   |    50 | o_agor@gmail.com                                   | 7055566774 | 117 |      11
  116 | Adams Bree      |    33 | a_bree@gmail.com                                   | 8056765424 | 102 |       1
  117 | Okafor Mathias  |    45 | o_mathias@gmail.com                                | 8056763367 | 120 |      10
  118 | Samson Adeleke  |    65 | s_adeleke@gmail.com                                | 7056774423 | 117 |      11
  119 | Lawal Tamire    |    35 | l_tamire@gmail.com                                 | 9052111101 | 107 |       5
  120 | James Job       |    44 | j_job@gmail.com                                    | 8059693919 | 100 |       8
  121 | Matthew Jakande |    21 | m_jakande@gmail.com                                | 7051232144 | 120 |       2
  122 | Jimila Adegboye |    20 | j_adegboye@gmail.com                               | 8054921923 | 107 |       5
(13 rows)


globacom_dbase=# SELECT * FROM DATAPLAN;
 data_id |                     data_size                      | data_duration | data_price
---------+----------------------------------------------------+---------------+------------
       1 | 350MB                                              |             2 |        200
       2 | 1.8GB                                              |            14 |        500
       3 | 3.9GB                                              |            30 |       1000
       4 | 7.5GB                                              |            30 |       1500
       5 | 9.2GB                                              |            30 |       2000
       6 | 10.8GB                                             |            30 |       2500
       7 | 14GB                                               |            30 |       3000
       8 | 18GB                                               |            30 |       4000
       9 | 24GB                                               |            30 |       5000
      10 | 29.9GB                                             |            30 |       8000
      11 | 50GB                                               |            30 |      10000
(11 rows)


globacom_dbase=# SELECT * FROM STAFF;
 staff_id |                     staff_name                     | no | staff_sal | age |   mobile
----------+----------------------------------------------------+----+-----------+-----+------------
      100 | Mustapha Ali                                       |  3 |    175000 |  32 | 8063285831
      107 | Alokwe Martin                                      |  7 |    380000 |  48 | 7090082812
       97 | Dankade  Aminat                                    |  5 |    550000 |  40 | 9023688832
      108 | Josiah Joshua                                      |  1 |    120000 |  30 | 8053189131
      102 | Mankinde Mary                                      |  2 |    450000 |  55 | 9023487830
      120 | Adeleke Jane                                       |  4 |    200000 |  38 | 7061045862
      122 | Osahon Mark                                        |  6 |    320000 |  44 | 802289842
      117 | Suleman Ajayi                                      |  3 |    800000 |  50 | 7030089811
      104 | Kuti Lawal                                         |  1 |    750000 |  35 | 9145689842