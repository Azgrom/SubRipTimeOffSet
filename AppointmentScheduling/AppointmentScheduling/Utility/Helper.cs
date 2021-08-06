using Microsoft.AspNetCore.Mvc.Rendering;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace AppointmentScheduling.Utility
{
    /// <summary>
    /// 
    /// </summary>
    public static class Helper
    {
        public static string Admin { get; set; } = "Admin";
        public static string Patient { get; set; } = "Patient";
        public static string Doctor { get; set; } = "Doctor";
        public static string AppointmentAdded { get; set; } = "Appointment added successfully.";
        public static string AppointmentUpdated { get; set; } = "Appointment updated successfully.";
        public static string AppointmentDeleted { get; set; } = "Appointment deleted successfully.";
        public static string AppointmentExists { get; set; } = "Appointment for selected date and time already exists.";
        public static string AppointmentNotExists { get; set; } = "Appointment not exists.";
        public static string AppointmentAddError { get; set; } = "Something went wrong, please try again.";
        public static string AppointmentUpdateError { get; set; } = "Something went wrong, please try again.";
        public static string SomethingWentWrong { get; set; } = "Something went wrong, please try again.";
        public static int SuccessCode { get; set; } = 1;
        public static int FailureCode { get; set; } = 0;

        public static IEnumerable<SelectListItem> GetRolesForDropDown()
        {
            return new List<SelectListItem>
            {
                new SelectListItem{Value = Helper.Admin, Text = Helper.Admin},
                new SelectListItem{Value = Helper.Patient, Text = Helper.Patient},
                new SelectListItem{Value = Helper.Doctor, Text = Helper.Doctor},
            };
        }

        public static List<SelectListItem> GetTimeDropDown()
        {
            var minute = 60;
            List<SelectListItem> duration = new();

            for (var i = 1; i <= 12; i++)
            {
                duration.Add(new SelectListItem { Value = minute.ToString(), Text = i + " Hr" });
                minute += 30;
                duration.Add(new SelectListItem { Value = minute.ToString(), Text = i + " Hr 30 min" });
                minute += 30;
            }

            return duration;
        }
    }
}
