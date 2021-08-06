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
        /// <summary>
        /// 
        /// </summary>
        private readonly ApplicationDbContext _db;

        /// <summary>
        /// 
        /// </summary>
        /// <param name="db"></param>
        public AppointmentService(ApplicationDbContext db)
        {
            _db = db;
        }

        /// <summary>
        /// 
        /// </summary>
        /// <param name="model"></param>
        /// <returns></returns>
        public async Task<int> AddUpdate(AppointmentViewModel model)
        {
            var startDate = DateTime.Parse(model.StartDate);
            var endDate = DateTime.Parse(model.StartDate).AddMinutes(Convert.ToDouble(model.Duration));

            if (model != null && model.Id > 0)
            {
                // Update appointment routine
                return 1;
            }
            else
            {
                // Create appointment routine
                var appointment = new Appointment()
                {
                    Title = model.Title,
                    Description = model.Description,
                    StartDate = startDate,
                    EndDate = endDate,
                    Duration = model.Duration,
                    DoctorId = model.DoctorId,
                    PatientId = model.PatientId,
                    IsDoctorApproved = false,
                    AdminId = model.AdminId
                };

                _db.Appointments.Add(appointment);
                await _db.SaveChangesAsync();

                return 2;
            }
        }

        /// <summary>
        /// 
        /// </summary>
        /// <returns></returns>
        public List<DoctorViewModel> GetDoctorList()
        {
            List<DoctorViewModel> doctors;
            doctors = (from user in _db.Users
                    join userRoles in _db.UserRoles on user.Id equals userRoles.UserId
                    join roles in _db.Roles.Where(x => x.Name == Helper.Doctor) on userRoles.RoleId equals roles.Id
                    select new DoctorViewModel
                    {
                        Id = user.Id,
                        Name = user.Name
                    }
                ).ToList();

            return doctors;
        }

        /// <summary>
        /// 
        /// </summary>
        /// <returns></returns>
        public List<PatientViewModel> GetPatientList()
        {
            List<PatientViewModel> patients;
            patients = (from user in _db.Users
                    join userRoles in _db.UserRoles on user.Id equals userRoles.UserId
                    join roles in _db.Roles.Where(x => x.Name == Helper.Patient) on userRoles.RoleId equals roles.Id
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
