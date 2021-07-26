using AppointmentScheduling.Services;
using Microsoft.AspNetCore.Http;
using Microsoft.AspNetCore.Mvc;
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
        public IActionResult SaveCalendarData()
        {
            CommonResponse<int> common_response = new CommonResponse<int>();
            try
            {
                common_response.status = _appointment_service.AddUpdate(data).Result;
                if (common_response.status == 1)
                {
                    common_response.message = Helper.appointment_updated;
                }
                if (common_response.status == 2)
                {
                    common_response.message = Helper.appointment_added;
                }
            }
            catch (Exception e)
            {
                common_response.message = e.Message;
                common_response.status = Helper.failure_code;
            }

            return Ok(common_response);
        }
    }
}
