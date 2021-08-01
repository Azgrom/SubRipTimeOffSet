using Microsoft.AspNetCore.Mvc.Rendering;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace AppointmentScheduling.Utility
{
    public class Helper
    {
        public static string Admin { get; set; } = "Admin";
        public static string Patient { get; set; } = "Patient";
        public static string Doctor { get; set; } = "Doctor";
        public static string Appointment_added { get; set; } = "Appointment added successfully.";
        public static string Appointment_updated { get; set; } = "Appointment updated successfully.";
        public static string Appointment_deleted { get; set; } = "Appointment deleted successfully.";
        public static string Appointment_exists { get; set; } = "Appointment for selected date and time already exists.";
        public static string Appointment_not_exists { get; set; } = "Appointment not exists.";
        public static string Appointment_add_error { get; set; } = "Sometring went wrong, please try again.";
        public static string Appointment_update_error { get; set; } = "Sometring went wrong, please try again.";
        public static string Something_went_wrong { get; set; } = "Sometring went wrong, please try again.";
        public static int Success_code { get; set; } = 1;
        public static int Failure_code { get; set; } = 0;

        public static List<SelectListItem> GetRolesForDropDown()
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
            int minute = 60;
            List<SelectListItem> duration = new();

            for (int i = 1; i <= 12; i++)
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
