program main
  use day01, only: day01_parts_1_and_2
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
  case default
    print *, "Code for day not found"
  end select
end program main
