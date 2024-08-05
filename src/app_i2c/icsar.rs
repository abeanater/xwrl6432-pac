#[doc = "Register `ICSAR` reader"]
pub type R = crate::R<IcsarSpec>;
#[doc = "Register `ICSAR` writer"]
pub type W = crate::W<IcsarSpec>;
#[doc = "Field `A9_A0` reader - 9:0\\]
Slave address. Use in both 7- and 10-bit address mode."]
pub type A9A0R = crate::FieldReader<u16>;
#[doc = "Field `A9_A0` writer - 9:0\\]
Slave address. Use in both 7- and 10-bit address mode."]
pub type A9A0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `NU` reader - 31:10\\]
Reserved"]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - 31:10\\]
Reserved"]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Slave address. Use in both 7- and 10-bit address mode."]
    #[inline(always)]
    pub fn a9_a0(&self) -> A9A0R {
        A9A0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Reserved"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Slave address. Use in both 7- and 10-bit address mode."]
    #[inline(always)]
    #[must_use]
    pub fn a9_a0(&mut self) -> A9A0W<IcsarSpec> {
        A9A0W::new(self, 0)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<IcsarSpec> {
        NuW::new(self, 10)
    }
}
#[doc = "I2C Slave Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`icsar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icsar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcsarSpec;
impl crate::RegisterSpec for IcsarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icsar::R`](R) reader structure"]
impl crate::Readable for IcsarSpec {}
#[doc = "`write(|w| ..)` method takes [`icsar::W`](W) writer structure"]
impl crate::Writable for IcsarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICSAR to value 0"]
impl crate::Resettable for IcsarSpec {
    const RESET_VALUE: u32 = 0;
}
