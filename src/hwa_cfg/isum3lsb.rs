#[doc = "Register `ISUM3LSB` reader"]
pub type R = crate::R<Isum3lsbSpec>;
#[doc = "Register `ISUM3LSB` writer"]
pub type W = crate::W<Isum3lsbSpec>;
#[doc = "Field `ISUM3LSB` reader - 31:0\\]
Refer ISUM1LSB"]
pub type Isum3lsbR = crate::FieldReader<u32>;
#[doc = "Field `ISUM3LSB` writer - 31:0\\]
Refer ISUM1LSB"]
pub type Isum3lsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Refer ISUM1LSB"]
    #[inline(always)]
    pub fn isum3lsb(&self) -> Isum3lsbR {
        Isum3lsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Refer ISUM1LSB"]
    #[inline(always)]
    #[must_use]
    pub fn isum3lsb(&mut self) -> Isum3lsbW<Isum3lsbSpec> {
        Isum3lsbW::new(self, 0)
    }
}
#[doc = "ISUM3LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`isum3lsb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isum3lsb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Isum3lsbSpec;
impl crate::RegisterSpec for Isum3lsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isum3lsb::R`](R) reader structure"]
impl crate::Readable for Isum3lsbSpec {}
#[doc = "`write(|w| ..)` method takes [`isum3lsb::W`](W) writer structure"]
impl crate::Writable for Isum3lsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISUM3LSB to value 0"]
impl crate::Resettable for Isum3lsbSpec {
    const RESET_VALUE: u32 = 0;
}
