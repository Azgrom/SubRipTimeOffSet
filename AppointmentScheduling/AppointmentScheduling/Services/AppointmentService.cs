using AppointmentScheduling.Models;
using AppointmentScheduling.Models.ViewModels;
using AppointmentScheduling.Utility;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace AppointmentScheduling.Services
{
    public class AppointmentService : IAppointmentService
    {
        private readonly ApplicationDbContext _db;

        public AppointmentService(ApplicationDbContext db)
        {
            _db = db;
        }

        public async Task<int> AddUpdate(AppointmentViewModel model)
        {
            var start_date = DateTime.Parse(model.StartDate);
            var end_date = DateTime.Parse(model.StartDate).AddMinutes(Convert.ToDouble(model.Duration));

            if (model != null && model.Id > 0)
            {
                // Update appointment routine
            }
            else
            {
                // Create appointment routine
                var appointment = new Appointment()
                {
                    Title = model.Title,
                    Description = model.Description,
                    StartDate = start_date,
                    EndDate = end_date,
                    Duration = model.Duration,
                    DoctorId = model.DoctorId,
                    PatientId = model.PatientId,
                    IsDoctorApproved = false,
                    AdminId = model.AdminId
                };

                _db.Appointment.Add(appointment);
                await _db.SaveChanges();

                return 2;
            }
        }

        public List<DoctorViewModel> GetDoctorList()
        {
            var doctors = (from user in _db.Users
                           join user_roles in _db.UserRoles on user.Id equals user_roles.UserId
                           join roles in _db.Roles.Where(x => x.Name == Helper.Doctor) on user_roles.RoleId equals roles.Id
                           select new DoctorViewModel
                           {
                               Id = user.Id,
                               Name = user.Name
                           }
                           ).ToList();

            return doctors;
        }

        public List<PatientViewModel> GetPatientList()
        {
            var patients = (from user in _db.Users
                           join user_roles in _db.UserRoles on user.Id equals user_roles.UserId
                           join roles in _db.Roles.Where(x => x.Name == Helper.Patient) on user_roles.RoleId equals roles.Id
                           select new PatientViewModel
                           {
                               Id = user.Id,
                               Name = user.Name
                           }
                           ).ToList();

            return patients;
        }
    }
}
