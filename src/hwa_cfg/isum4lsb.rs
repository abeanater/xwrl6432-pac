#[doc = "Register `ISUM4LSB` reader"]
pub type R = crate::R<Isum4lsbSpec>;
#[doc = "Register `ISUM4LSB` writer"]
pub type W = crate::W<Isum4lsbSpec>;
#[doc = "Field `ISUM4LSB` reader - 31:0\\]
Refer ISUM1LSB"]
pub type Isum4lsbR = crate::FieldReader<u32>;
#[doc = "Field `ISUM4LSB` writer - 31:0\\]
Refer ISUM1LSB"]
pub type Isum4lsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Refer ISUM1LSB"]
    #[inline(always)]
    pub fn isum4lsb(&self) -> Isum4lsbR {
        Isum4lsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Refer ISUM1LSB"]
    #[inline(always)]
    #[must_use]
    pub fn isum4lsb(&mut self) -> Isum4lsbW<Isum4lsbSpec> {
        Isum4lsbW::new(self, 0)
    }
}
#[doc = "ISUM4LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`isum4lsb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isum4lsb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Isum4lsbSpec;
impl crate::RegisterSpec for Isum4lsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isum4lsb::R`](R) reader structure"]
impl crate::Readable for Isum4lsbSpec {}
#[doc = "`write(|w| ..)` method takes [`isum4lsb::W`](W) writer structure"]
impl crate::Writable for Isum4lsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISUM4LSB to value 0"]
impl crate::Resettable for Isum4lsbSpec {
    const RESET_VALUE: u32 = 0;
}
