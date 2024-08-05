#[doc = "Register `PID` reader"]
pub type R = crate::R<PidSpec>;
#[doc = "Register `PID` writer"]
pub type W = crate::W<PidSpec>;
#[doc = "Field `PID_W6_O0` reader - "]
pub type PidW6O0R = crate::FieldReader;
#[doc = "Field `PID_W6_O0` writer - "]
pub type PidW6O0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PID_W2_O6` reader - "]
pub type PidW2O6R = crate::FieldReader;
#[doc = "Field `PID_W2_O6` writer - "]
pub type PidW2O6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PID_W3_O8` reader - "]
pub type PidW3O8R = crate::FieldReader;
#[doc = "Field `PID_W3_O8` writer - "]
pub type PidW3O8W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PID_W5_O11` reader - "]
pub type PidW5O11R = crate::FieldReader;
#[doc = "Field `PID_W5_O11` writer - "]
pub type PidW5O11W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PID_W16_O16` reader - "]
pub type PidW16O16R = crate::FieldReader<u16>;
#[doc = "Field `PID_W16_O16` writer - "]
pub type PidW16O16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn pid_w6_o0(&self) -> PidW6O0R {
        PidW6O0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn pid_w2_o6(&self) -> PidW2O6R {
        PidW2O6R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pid_w3_o8(&self) -> PidW3O8R {
        PidW3O8R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn pid_w5_o11(&self) -> PidW5O11R {
        PidW5O11R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn pid_w16_o16(&self) -> PidW16O16R {
        PidW16O16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn pid_w6_o0(&mut self) -> PidW6O0W<PidSpec> {
        PidW6O0W::new(self, 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn pid_w2_o6(&mut self) -> PidW2O6W<PidSpec> {
        PidW2O6W::new(self, 6)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn pid_w3_o8(&mut self) -> PidW3O8W<PidSpec> {
        PidW3O8W::new(self, 8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    #[must_use]
    pub fn pid_w5_o11(&mut self) -> PidW5O11W<PidSpec> {
        PidW5O11W::new(self, 11)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn pid_w16_o16(&mut self) -> PidW16O16W<PidSpec> {
        PidW16O16W::new(self, 16)
    }
}
#[doc = "PID register\n\nYou can [`read`](crate::Reg::read) this register and get [`pid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PidSpec;
impl crate::RegisterSpec for PidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pid::R`](R) reader structure"]
impl crate::Readable for PidSpec {}
#[doc = "`write(|w| ..)` method takes [`pid::W`](W) writer structure"]
impl crate::Writable for PidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PID to value 0"]
impl crate::Resettable for PidSpec {
    const RESET_VALUE: u32 = 0;
}
