using AppointmentScheduling.Models.ViewModels;
using AppointmentScheduling.Services;
using AppointmentScheduling.Utility;
using Microsoft.AspNetCore.Http;
using Microsoft.AspNetCore.Mvc;
using System;
using System.Security.Claims;

namespace AppointmentScheduling.Controllers
{
    [Route("API/Appointment")]
    [ApiController]
    public class AppointmentAPIController : Controller
    {
        private readonly IAppointmentService _appointment_service;
        private readonly IHttpContextAccessor _http_context_accessor;
        private readonly string login_user_id;
        private readonly string role;

        public AppointmentAPIController(IAppointmentService appointment_service, 
                                        IHttpContextAccessor http_context_accessor)
        {
            _appointment_service = appointment_service;
            _http_context_accessor = http_context_accessor;
            login_user_id = _http_context_accessor.HttpContext.User.FindFirstValue(ClaimTypes.NameIdentifier);
            role = _http_context_accessor.HttpContext.User.FindFirstValue(ClaimTypes.Role);
        }

        [HttpPost]
        [Route("SaveCalendarData")]
        public IActionResult SaveCalendarData(AppointmentViewModel data)
        {
            CommonResponse<int> common_response = new CommonResponse<int>();
            try
            {
                common_response.Status = _appointment_service.AddUpdate(data).Result;

                common_response.Message = common_response.Status switch
                {
                    1 => Helper.AppointmentUpdated,
                    2 => Helper.AppointmentAdded,
                    _ => Helper.SomethingWentWrong,
                };
            }
            catch (Exception e)
            {
                common_response.Message = e.Message;
                common_response.Status = Helper.FailureCode;
            }

            return Ok(common_response);
        }
    }
}
