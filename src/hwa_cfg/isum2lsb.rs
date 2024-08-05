#[doc = "Register `ISUM2LSB` reader"]
pub type R = crate::R<Isum2lsbSpec>;
#[doc = "Register `ISUM2LSB` writer"]
pub type W = crate::W<Isum2lsbSpec>;
#[doc = "Field `ISUM2LSB` reader - 31:0\\]
Refer ISUM1LSB"]
pub type Isum2lsbR = crate::FieldReader<u32>;
#[doc = "Field `ISUM2LSB` writer - 31:0\\]
Refer ISUM1LSB"]
pub type Isum2lsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Refer ISUM1LSB"]
    #[inline(always)]
    pub fn isum2lsb(&self) -> Isum2lsbR {
        Isum2lsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Refer ISUM1LSB"]
    #[inline(always)]
    #[must_use]
    pub fn isum2lsb(&mut self) -> Isum2lsbW<Isum2lsbSpec> {
        Isum2lsbW::new(self, 0)
    }
}
#[doc = "ISUM2LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`isum2lsb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isum2lsb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Isum2lsbSpec;
impl crate::RegisterSpec for Isum2lsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isum2lsb::R`](R) reader structure"]
impl crate::Readable for Isum2lsbSpec {}
#[doc = "`write(|w| ..)` method takes [`isum2lsb::W`](W) writer structure"]
impl crate::Writable for Isum2lsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISUM2LSB to value 0"]
impl crate::Resettable for Isum2lsbSpec {
    const RESET_VALUE: u32 = 0;
}
