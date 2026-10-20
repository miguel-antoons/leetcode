with emails as (
    select distinct email, count(id) as c
    from Person
    group by email
)
select email
from emails
where c > 1;


select email
from Person
group by email
having count(email) > 1;