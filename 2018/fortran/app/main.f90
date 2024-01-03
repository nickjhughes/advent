program main
  use day01, only: day01_parts_1_and_2
  use day02, only: day02_parts_1_and_2
  use day03, only: day03_parts_1_and_2
  use day04, only: day04_parts_1_and_2
  use day05, only: day05_parts_1_and_2
  use day06, only: day06_parts_1_and_2
  implicit none

  integer :: num_args, day
  character(len=2) :: day_arg

  num_args = command_argument_count()
  if (num_args == 0) then
    write(0,*) "Please specify a day"
    return
  end if
  call get_command_argument(1, day_arg)
  read(day_arg, '(I2)') day
  
  select case(day)
  case(1)
    call day01_parts_1_and_2
  case(2)
    call day02_parts_1_and_2
  case(3)
    call day03_parts_1_and_2
  case(4)
    call day04_parts_1_and_2
  case(5)
    call day05_parts_1_and_2
  case(6)
    call day06_parts_1_and_2
  case default
    print *, "Code for day not found"
  end select
end program main
