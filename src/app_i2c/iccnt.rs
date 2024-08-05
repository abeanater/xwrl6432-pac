#[doc = "Register `ICCNT` reader"]
pub type R = crate::R<IccntSpec>;
#[doc = "Register `ICCNT` writer"]
pub type W = crate::W<IccntSpec>;
#[doc = "Field `ICDC15_ICDC0` reader - 15:0\\]
Data count. This data count register is used to generate a Stop condition if a Stop condition is specified (STP=1). . ICCNT=1 data count is 1 :::::::::::::::::::::::::::::::::::::::::::: :::::::::::::::::::::::::::::::::::::::::::: ICCNT=0FFFFh data count is 65535 ICCNT=0data counter is 65536 Note that ICCNT is a don\"t care when RM is set to 1."]
pub type Icdc15Icdc0R = crate::FieldReader<u16>;
#[doc = "Field `ICDC15_ICDC0` writer - 15:0\\]
Data count. This data count register is used to generate a Stop condition if a Stop condition is specified (STP=1). . ICCNT=1 data count is 1 :::::::::::::::::::::::::::::::::::::::::::: :::::::::::::::::::::::::::::::::::::::::::: ICCNT=0FFFFh data count is 65535 ICCNT=0data counter is 65536 Note that ICCNT is a don\"t care when RM is set to 1."]
pub type Icdc15Icdc0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `NU` reader - 31:16\\]
Reserved"]
pub type NuR = crate::FieldReader<u16>;
#[doc = "Field `NU` writer - 31:16\\]
Reserved"]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Data count. This data count register is used to generate a Stop condition if a Stop condition is specified (STP=1). . ICCNT=1 data count is 1 :::::::::::::::::::::::::::::::::::::::::::: :::::::::::::::::::::::::::::::::::::::::::: ICCNT=0FFFFh data count is 65535 ICCNT=0data counter is 65536 Note that ICCNT is a don\"t care when RM is set to 1."]
    #[inline(always)]
    pub fn icdc15_icdc0(&self) -> Icdc15Icdc0R {
        Icdc15Icdc0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Data count. This data count register is used to generate a Stop condition if a Stop condition is specified (STP=1). . ICCNT=1 data count is 1 :::::::::::::::::::::::::::::::::::::::::::: :::::::::::::::::::::::::::::::::::::::::::: ICCNT=0FFFFh data count is 65535 ICCNT=0data counter is 65536 Note that ICCNT is a don\"t care when RM is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn icdc15_icdc0(&mut self) -> Icdc15Icdc0W<IccntSpec> {
        Icdc15Icdc0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<IccntSpec> {
        NuW::new(self, 16)
    }
}
#[doc = "I2C Data Count register\n\nYou can [`read`](crate::Reg::read) this register and get [`iccnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iccnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IccntSpec;
impl crate::RegisterSpec for IccntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iccnt::R`](R) reader structure"]
impl crate::Readable for IccntSpec {}
#[doc = "`write(|w| ..)` method takes [`iccnt::W`](W) writer structure"]
impl crate::Writable for IccntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICCNT to value 0"]
impl crate::Resettable for IccntSpec {
    const RESET_VALUE: u32 = 0;
}
