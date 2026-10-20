with em_man as (
    select e.name as name, e.salary as e_salary, m.salary as m_salary
    from Employee e
    join Employee m on e.managerId = m.id
)
select name as Employee
from em_man
where e_salary > m_salary;

select e.name as Employee
from Employee e
join Employee m on e.managerId = m.id
where e.salary > m.salary;