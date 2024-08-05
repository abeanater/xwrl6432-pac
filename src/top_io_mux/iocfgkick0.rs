#[doc = "Register `IOCFGKICK0` reader"]
pub type R = crate::R<Iocfgkick0Spec>;
#[doc = "Register `IOCFGKICK0` writer"]
pub type W = crate::W<Iocfgkick0Spec>;
#[doc = "Field `IOCFGKICK0` reader - 31:0\\]
Kicker 0 Register. The value 83E7 0B13h must be written to KICK0 as part of the process to unlock the CPU write access to the above PIN MUX registers (including IOCFGKICK1)"]
pub type Iocfgkick0R = crate::FieldReader<u32>;
#[doc = "Field `IOCFGKICK0` writer - 31:0\\]
Kicker 0 Register. The value 83E7 0B13h must be written to KICK0 as part of the process to unlock the CPU write access to the above PIN MUX registers (including IOCFGKICK1)"]
pub type Iocfgkick0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Kicker 0 Register. The value 83E7 0B13h must be written to KICK0 as part of the process to unlock the CPU write access to the above PIN MUX registers (including IOCFGKICK1)"]
    #[inline(always)]
    pub fn iocfgkick0(&self) -> Iocfgkick0R {
        Iocfgkick0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Kicker 0 Register. The value 83E7 0B13h must be written to KICK0 as part of the process to unlock the CPU write access to the above PIN MUX registers (including IOCFGKICK1)"]
    #[inline(always)]
    #[must_use]
    pub fn iocfgkick0(&mut self) -> Iocfgkick0W<Iocfgkick0Spec> {
        Iocfgkick0W::new(self, 0)
    }
}
#[doc = "IOCFGKICK0\n\nYou can [`read`](crate::Reg::read) this register and get [`iocfgkick0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iocfgkick0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iocfgkick0Spec;
impl crate::RegisterSpec for Iocfgkick0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iocfgkick0::R`](R) reader structure"]
impl crate::Readable for Iocfgkick0Spec {}
#[doc = "`write(|w| ..)` method takes [`iocfgkick0::W`](W) writer structure"]
impl crate::Writable for Iocfgkick0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOCFGKICK0 to value 0"]
impl crate::Resettable for Iocfgkick0Spec {
    const RESET_VALUE: u32 = 0;
}
